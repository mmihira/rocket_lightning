import React from 'react';
import {connect} from 'react-redux';
import PropTypes from 'prop-types';
import act from 'actions';
import './styles/app.css';
import TopBar from 'features/TopBar';
import * as d3 from 'd3';

function foo () {
  const color = 'steelblue';
  const height = 500;
  const width = 800;
  const margin = {top: 20, right: 20, bottom: 30, left: 40};
  // const formatCount = d3.format(',.0f');

  const data = d3.range(1000).map(d3.randomNormal(20, 5));

  const x = d3.scaleLinear()
    .domain(d3.extent(data))
    .range([0, width]);

  const bins = d3.histogram()
    .domain(x.domain())
    .thresholds(x.ticks(40))(data);

  const yMax = d3.max(bins, _x => _x.length);
  const yMin = d3.min(bins, _x => _x.length);
  const colorScale = d3.scaleLinear()
    .domain([yMin, yMax])
    .range([d3.rgb(color).brighter(), d3.rgb(color).darker()]);

  const y = d3.scaleLinear()
    .domain([0, d3.max(bins, d => d.length)])
    .range([height, 0]);

  const xAxis = d3.axisBottom(x);

  const svg = d3.select('#chartEl').append('svg')
    .attr('width', width + margin.left + margin.right)
    .attr('height', height + margin.top + margin.bottom)
    .append('g')
    .attr('transform', 'translate(' + margin.left + ',' + margin.top + ')');

  const bar = svg.selectAll('.bar')
    .data(bins)
    .enter().append('g')
    .attr('class', 'bar')
    .attr('transform', function (d) { return 'translate(' + x(d.x0) + ',' + y(d.length) + ')'; });

  bar.append('rect')
    .attr('x', 1)
    .attr('width', d=> x(d.x1) - x(d.x0) - 1)
    .attr('height', d => height - y(d.length))
    .attr('fill', d => colorScale(d.length));

  // bar.append('text')
  //   .attr('dy', '.75em')
  //   .attr('y', 20)
  //   .attr('x', d => (x(d.x1) - x(d.x0)) / 2)
  //   .attr('text-anchor', 'middle')
  //   .text(d => formatCount(d.x0));

  svg.append('g')
    .attr('class', 'x axis')
    .attr('transform', 'translate(0,' + height + ')')
    .call(xAxis);
}

class App extends React.Component {
  constructor (props) {
    super(props);
    this.x = foo;
  }

  componentDidMount () {
    foo();
  }

  componentWillReceiveProps (nextProps) {
    nextProps;
  }

  render () {
    return(
      <div>
        <TopBar/>
        <div id="chartEl" />
      </div>
    );
  }
}

const mapDispatchToProps = dispatch => {
  return {
    setAppDimensions: (width, height) => dispatch(act.setAppDimensions(width, height)),
  };
};

const mapStateToProps = state => {
  state;
  return {};
};

App.propTypes = {
  setAppDimensions: PropTypes.func
};

export default connect(mapStateToProps, mapDispatchToProps)(App);
